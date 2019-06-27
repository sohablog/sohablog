var setStatus = function (id, status, callback, eallback) {
	var data={};
	data[window.__injectData.tokenFieldName]=window.__injectData.token;
	$.ajax({
		url: window.__injectData.setStatusUrl.replace('-20001003', id).replace('-20000309', status),
	//   ^magic numbers, seems no better idea when using `url!` generator
		type: 'POST',
		data: data,
		success: function (){
			!callback || callback();
		},
		error: function (e){
			console.error(e);
			!eallback || eallback(e);
		}
	});
};

$(document).ready(function (){
	$('.set-status-buttons').each(function (){
		var commentId = $(this).data('id');
		var previousDisabledButton = $('button[disabled]', this);
		$('button', this).each(function (){
			var statusId = $(this).data('status');
			$(this).on('click', function (){
				$(this).attr('disabled', true);
				var thisButton = $(this);
				setStatus(commentId, statusId, function (){
					previousDisabledButton.attr('disabled', false);
					previousDisabledButton = thisButton;
				}, function (){
					$(this).attr('disabled', false);
					alert('Update status failed!');
				});
			});
		});
	});
});
