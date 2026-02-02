const MtrDatepickerDemo = (function () {
  const datepickers = [];
  let exportSettings;

  const init = function (config, settings) {
    exportSettings = settings;
    const datepicker = new MtrDatepicker(config);
    datepickers.push(datepicker);

    const exportFormatsContainer = document.getElementById(settings.exportFormats);
	 	datepickerChange(exportFormatsContainer, datepicker, settings.exportFormats);

	 	datepicker.onChange('all', function () {
      datepickerChange(exportFormatsContainer, datepicker, settings.exportFormats);
    });

    return datepicker;
  };

  function datepickerChange (resultElement, datepicker, elemId) {
    const result = datepicker.format('YYYY-MM-DD HH:mm');
    if (elemId == 'datepicker-1-res') {
      $('#id_start_date').val(result);
    } else if (elemId == 'datepicker-2-res') {
      $('#id_end_date').val(result);
    }
    resultElement.innerHTML = result;
  }

  return {
    init
  };
})();
