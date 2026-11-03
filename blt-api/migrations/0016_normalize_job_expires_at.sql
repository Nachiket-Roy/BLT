-- Migration number: 0016   2026-11-04T00:00:00.000Z
-- Normalize job expires_at timestamps to 'YYYY-MM-DD HH:MM:SS' format.

UPDATE jobs
SET expires_at = strftime('%Y-%m-%d %H:%M:%S', expires_at)
WHERE expires_at IS NOT NULL;

