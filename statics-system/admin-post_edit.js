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
