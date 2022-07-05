var header = `
<div class="card mb-3">
    <div class="row g-0">   
        <div class="col-md-2">
            <!-- for image -->
        </div>
        <div class="col-md-10">
            <div class="card-body">`;

var footer =  `</div></div></div></div></a>`;

var str = "";
window.looper.forEach((slide, reversed_index) => {
    var index = window.looper.length - reversed_index - 1;
    str += '<a href="#" class="no_link" id="href-' + index + '">'
        + header
        + '<h5 class="card-title" id="title-' + index + '">' + slide + '</h5>' 
        + '<p class="card-text" id="author-' + index + '">' + '</p>'
        + '<p class="card-text" id="text-' + index + '">' + '</p>'
        + footer;
});

// str += footer;
document.getElementById("variousBlogs").innerHTML = str;

window.looper.forEach((value, reversed_index) => {
    window.get_nft(value).then(data => {
        var index = window.looper.length - reversed_index - 1;
        document.getElementById("href-" + index).href = "/article/" + index;
        document.getElementById("title-" + index).innerText = data.name;
        document.getElementById("author-" + index).innerText = data.properties.author;

        var text = data.properties.content["text/markdown"];
        var arr = text.split("\n");
        var result = arr.splice(0, 1);
        result.push(arr.join(" "));
        var text = result[1];
        document.getElementById("text-" + index).innerText = text.substr(0, 100);
    });
});