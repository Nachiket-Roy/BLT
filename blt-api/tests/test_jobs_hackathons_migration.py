import sqlite3
from pathlib import Path

import pytest


MIGRATION_SQL = Path(__file__).resolve().parents[1] / "migrations" / "0012_jobs_hackathons.sql"
NORMALIZE_MIGRATION_SQL = Path(__file__).resolve().parents[1] / "migrations" / "0016_normalize_job_expires_at.sql"


def _migrated_db():
    conn = sqlite3.connect(":memory:")
    conn.execute("PRAGMA foreign_keys = ON")
    conn.execute("CREATE TABLE users (id INTEGER PRIMARY KEY)")
    conn.executemany("INSERT INTO users (id) VALUES (?)", [(1,), (2,)])
    conn.executescript(MIGRATION_SQL.read_text(encoding="utf-8"))
    return conn


def test_hackathons_migration_rejects_invalid_date_range():
    conn = _migrated_db()

    with pytest.raises(sqlite3.IntegrityError, match="hackathons_valid_dates_check"):
        conn.execute(
            """
            INSERT INTO hackathons (
                created_by_user_id,
                title,
                slug,
                starts_at,
                ends_at
            )
            VALUES (?, ?, ?, ?, ?)
            """,
            (1, "Invalid Dates", "invalid-dates", "2026-06-12 00:00:00", "2026-06-10 00:00:00"),
        )


def test_hackathon_registration_insert_requires_matching_owner():
    conn = _migrated_db()
    cursor = conn.execute(
        """
        INSERT INTO hackathons (
            created_by_user_id,
            title,
            slug,
            starts_at,
            ends_at
        )
        VALUES (?, ?, ?, ?, ?)
        """,
        (1, "Owner Check", "owner-check", "2026-06-10 00:00:00", "2026-06-12 00:00:00"),
    )

    with pytest.raises(sqlite3.IntegrityError, match="hackathon registration owner mismatch"):
        conn.execute(
            """
            INSERT INTO hackathon_registrations (
                hackathon_id,
                created_by_user_id,
                participant_email_encrypted,
                participant_email_hash
            )
            VALUES (?, ?, ?, ?)
            """,
            (cursor.lastrowid, 2, "encrypted", "hash"),
        )


def test_hackathon_registration_update_requires_matching_owner():
    conn = _migrated_db()
    cursor = conn.execute(
        """
        INSERT INTO hackathons (
            created_by_user_id,
            title,
            slug,
            starts_at,
            ends_at
        )
        VALUES (?, ?, ?, ?, ?)
        """,
        (1, "Owner Update", "owner-update", "2026-06-10 00:00:00", "2026-06-12 00:00:00"),
    )
    registration = conn.execute(
        """
        INSERT INTO hackathon_registrations (
            hackathon_id,
            created_by_user_id,
            participant_email_encrypted,
            participant_email_hash
        )
        VALUES (?, ?, ?, ?)
        """,
        (cursor.lastrowid, 1, "encrypted", "hash"),
    )

    with pytest.raises(sqlite3.IntegrityError, match="hackathon registration owner mismatch"):
        conn.execute(
            "UPDATE hackathon_registrations SET created_by_user_id = ? WHERE id = ?",
            (2, registration.lastrowid),
        )


def test_normalize_job_expires_at_migration():
    conn = _migrated_db()
    conn.execute(
        """
        INSERT INTO jobs (
            created_by_user_id,
            title,
            description,
            expires_at
        )
        VALUES (1, 'Test Job 1', 'Desc', '2026-11-01T10:00:00Z'),
               (1, 'Test Job 2', 'Desc', '2026-11-05 10:00:00'),
               (1, 'Test Job 3', 'Desc', NULL)
        """
    )
    conn.executescript(NORMALIZE_MIGRATION_SQL.read_text(encoding="utf-8"))
    cursor = conn.execute("SELECT id, expires_at FROM jobs WHERE title LIKE 'Test Job%' ORDER BY id")
    rows = cursor.fetchall()
    assert rows[0][1] == "2026-11-01 10:00:00"
    assert rows[1][1] == "2026-11-05 10:00:00"
    assert rows[2][1] is None
