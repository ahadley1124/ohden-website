document.addEventListener('DOMContentLoaded', function() {
    document.getElementById('404submit').addEventListener('click', submit404);
});

function submit404() {
    //get the current URL
    var url = window.location.href;
    //post the URL to the server
    var xhr = new XMLHttpRequest();
    xhr.open('POST', '/404', true);
    xhr.setRequestHeader('Content-Type', 'application/json');
    xhr.send(JSON.stringify({url: url}));
    //redirect to the homepage
    window.location.href = '/';
}