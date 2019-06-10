var mpContent=new MarkdownPalettes("#content-mp");
document.getElementById('submit').onclick=function (){
	document.getElementById('content').value=mpContent.content;
	document.getElementById('save_draft').value='false';
	return true;
};
document.getElementById('submit_draft').onclick=function (){
	document.getElementById('content').value=mpContent.content;
	document.getElementById('save_draft').value='true';
	return true;
};
document.addEventListener('DOMContentLoaded',function (){
	mpContent.content=document.getElementById('content').value;
},false);

var fetchAllAttachments = function () {
	var ctn = $('#attachments-container');
	var uploadDir = ctn.data('upload-route');
	$.ajax({
		url: ctn.data('url'),
		type: 'GET',
		dataType: 'json',
		success: function (data){
			var root=$('<div id="list"></div>');
			data.forEach(function (v) {
				var e=$('<div><h4></h4><p><small></small></p><p><code></code></p><hr /></div>');
				$('h4', e).text(v.filename);
				$('small', e).text(v.time);
				var ext="";
				if(v.filename){
					ext=v.filename.split('.');
					ext=ext[ext.length-1].toLowerCase();
				}
				$('code', e).text((['jpg','jpeg','png','gif','svg','bmp'].indexOf(ext)<0?"":"!")+"["+v.filename+"]("+v.key.replace('{upload_dir}', uploadDir)+")");
				root.prepend(e);
			});
			$('#list', ctn).replaceWith(root);
		},
		error: function (e){
			console.error(e);
			alert('Fetch failed');
		}
	});
};

$(document).ready(function (){
	fetchAllAttachments();

	// file uploading handler
	$('#attachment-upload').on("submit", function (e){
		e.preventDefault();
		var form = $(e.currentTarget);
		var fileBox=$('input[name="file"]', e.currentTarget);
		var submitButton=$('input[type="submit"]', e.currentTarget);
		var formData = new FormData(e.currentTarget);

		if(fileBox[0].files.length !== 1){
			alert("expected files.length = 1, got "+fileBox[0].files.length);
		}

		submitButton.attr('disabled', true);
		$.ajax({
			url: form.attr('action'),
			type: 'POST',
			data: formData,
			contentType: false,
			processData: false,
			success: function (data){
				fileBox.replaceWith(fileBox.val('').clone());
				fetchAllAttachments();
			},
			error: function (e){
				console.error(e);
				alert('Upload failed');
			},
			complete: function (){
				submitButton.attr('disabled', false);
			}
		});
	});
});
