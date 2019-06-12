$(document).ready(function () {
	$('.reply-to-comment').on('click', function (e) {
		$('#comment-form').detach().insertAfter($(e.currentTarget));
		$('input[name="reply_to"]', '#comment-form').remove();
		var a=$('<input type="hidden" name="reply_to" />');
		a.val(e.currentTarget.dataset.id);
		a.appendTo('#comment-form');
		$('#cancel-reply').style('display', 'inherit');
	});
	$('#cancel-reply').on('click', function (e) {
		$(e.currentTarget).style('display', 'none');
		$('input[name="reply_to"]', '#comment-form').remove();
		$('#comment-form').detach().appendTo('#comment-form-wrapper');
	});
});
