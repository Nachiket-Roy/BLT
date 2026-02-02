(function () {
  'use strict';

  $('.userRoleListItem').click(function (e) {
    const name = e.currentTarget;
    $('#user_' + name.getAttribute('id')).text(name.getAttribute('data-name'));
    $('#role_' + name.getAttribute('id')).val(name.getAttribute('value'));
  });

  $('.userDomainListItem').click(function (e) {
    const name = e.currentTarget;
    $('#domain_' + name.getAttribute('id')).text(name.getAttribute('data-name'));
    $('#domain' + name.getAttribute('id')).val(name.getAttribute('value'));
  });

  $('#update-role').submit(function (e) {
    e.preventDefault();
    const serializedData = $(this).serialize();
    $.ajax({
      type: 'POST',
      url: '/dashboard/organization/settings/role/update',
      data: serializedData,
      success: function (response) {
        window.location.reload();
      },
      error: function (response) {
        // alert the error if any error occurred
        alert(response.responseJSON.error);
      }
    });
  });

  $('#update-hunt').submit(function (e) {
    e.preventDefault();

    const date1 = document.getElementById('datepicker-1-res').innerHTML;
    const date2 = document.getElementById('datepicker-2-res').innerHTML;
    const serializedData = $(this).serializeArray();
    const offset = new Date().getTimezoneOffset();
    serializedData.push({ name: 'tzoffset', value: offset });
    serializedData.push({ name: 'date1', value: date1 });
    serializedData.push({ name: 'date2', value: date2 });
    const value = ($(this).serializeArray())[1].value;
    $.ajax({
      type: 'POST',
      url: '/dashboard/organization/hunt/' + value + '/edit',
      data: serializedData,
      success: function (response) {
        window.location.reload();
      },
      error: function (response) {
        // alert the error if any error occurred
        alert(response.responseJSON.error);
      }
    });
  });

  $('#create-hunt').submit(function (e) {
    e.preventDefault();
    const date1 = document.getElementById('datepicker-1-res').innerHTML;
    const date2 = document.getElementById('datepicker-2-res').innerHTML;
    const serializedData = $(this).serializeArray();
    const offset = new Date().getTimezoneOffset();
    serializedData.push({ name: 'tzoffset', value: offset });
    serializedData.push({ name: 'date1', value: date1 });
    serializedData.push({ name: 'date2', value: date2 });
    $.ajax({
      type: 'POST',
      url: '/dashboard/organization/hunt/create',
      data: $.param(serializedData),
      success: function (response) {
        window.location.reload();
      },
      error: function (response) {
        // alert the error if any error occurred
        alert(response.responseJSON.error);
      }
    });
  });

  $('#add-role').submit(function (e) {
    e.preventDefault();
    const serializedData = $(this).serialize();
    $.ajax({
      type: 'POST',
      url: '/dashboard/organization/settings/role/add',
      data: serializedData,
      success: function (response) {
        window.location.reload();
      },
      error: function (response) {
        // alert the error if any error occurred
        alert(response.responseJSON.error);
      }
    });
  });

  $('#add-or-update-organization').submit(function (e) {
    e.preventDefault();
    const serializedData = $(this).serialize();
    $.ajax({
      type: 'POST',
      url: '/dashboard/admin/organization/addorupdate',
      data: serializedData,
      success: function (response) {
        window.location.reload();
      },
      error: function (response) {
        // alert the error if any error occurred
        alert(response.responseJSON.error);
      }
    });
  });

  $('#add-or-update-domain').submit(function (e) {
    e.preventDefault();
    const serializedData = $(this).serialize();
    $.ajax({
      type: 'POST',
      url: '/dashboard/organization/domain/addorupdate',
      data: serializedData,
      success: function (response) {
        window.location.reload();
      },
      error: function (response) {
        // alert the error if any error occurred
        alert(response.responseJSON.error);
      }
    });
  });

  // Toggle Left Menu
  jQuery('.menu-list > a').click(function () {
    const parent = jQuery(this).parent();
    const sub = parent.find('> ul');

    if (!jQuery('body').hasClass('left-side-collapsed')) {
      if (sub.is(':visible')) {
        sub.slideUp(200, function () {
          parent.removeClass('nav-active');
          jQuery('.main-content').css({ height: '' });
          mainContentHeightAdjust();
        });
      } else {
        visibleSubMenuClose();
        parent.addClass('nav-active');
        sub.slideDown(200, function () {
          mainContentHeightAdjust();
        });
      }
    }
    return false;
  });

  function visibleSubMenuClose () {
    jQuery('.menu-list').each(function () {
      const t = jQuery(this);
      if (t.hasClass('nav-active')) {
        t.find('> ul').slideUp(200, function () {
          t.removeClass('nav-active');
        });
      }
    });
  }

  function mainContentHeightAdjust () {
    // Adjust main content height
    const docHeight = jQuery(document).height();
    if (docHeight > jQuery('.main-content').height()) { jQuery('.main-content').height(docHeight); }
  }

  //  class add mouse hover
  jQuery('.custom-nav > li').hover(function () {
    jQuery(this).addClass('nav-hover');
  }, function () {
    jQuery(this).removeClass('nav-hover');
  });

  // Menu Toggle
  jQuery('.toggle-btn').click(function () {
    $('.left-side').getNiceScroll().hide();

    if ($('body').hasClass('left-side-collapsed')) {
      $('.left-side').getNiceScroll().hide();
    }
    const body = jQuery('body');
    const bodyposition = body.css('position');

    if (bodyposition != 'relative') {
      if (!body.hasClass('left-side-collapsed')) {
        body.addClass('left-side-collapsed');
        jQuery('.custom-nav ul').attr('style', '');

        jQuery(this).addClass('menu-collapsed');
      } else {
        body.removeClass('left-side-collapsed chat-view');
        jQuery('.custom-nav li.active ul').css({ display: 'block' });

        jQuery(this).removeClass('menu-collapsed');
      }
    } else {
      if (body.hasClass('left-side-show')) { body.removeClass('left-side-show'); } else { body.addClass('left-side-show'); }

      mainContentHeightAdjust();
    }
  });

  searchform_reposition();

  jQuery(window).resize(function () {
    if (jQuery('body').css('position') == 'relative') {
      jQuery('body').removeClass('left-side-collapsed');
    } else {
      jQuery('body').css({ left: '', marginRight: '' });
    }

    searchform_reposition();
  });

  function searchform_reposition () {
    if (jQuery('.searchform').css('position') == 'relative') {
      jQuery('.searchform').insertBefore('.left-side-inner .logged-user');
    } else {
      jQuery('.searchform').insertBefore('.menu-right');
    }
  }
})(jQuery);

// Dropdowns Script
$(document).ready(function () {
						  $(document).on('click', function (ev) {
						    ev.stopImmediatePropagation();
						    $('.dropdown-toggle').dropdown('active');
						  });
});

/** ************ Search ****************/
$(function () {
	    const button = $('#loginButton');
	    const box = $('#loginBox');
	    const form = $('#loginForm');
	    button.removeAttr('href');
	    button.mouseup(function (login) {
	        box.toggle();
	        button.toggleClass('active');
	    });
	    form.mouseup(function () {
	        return false;
	    });
	    $(this).mouseup(function (login) {
	        if (!($(login.target).parent('#loginButton').length > 0)) {
	            button.removeClass('active');
	            box.hide();
	        }
	    });
});
