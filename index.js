let a = "?b=3&a=2";
let b = a.match(/[\?|\&]b=(.*?)(\&|$)/)
console.log(b);