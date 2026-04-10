# Sumário

- getElementById
- getElementsByTagName
- getElementsByClassName

## getElementById

```html
<div id="a1">HTML</div>
<div id="a2">CSS</div>
<div id="a3">JAVASCRIPT</div>
```

```javascript
const a1 = document.getElementById("a1");
const a2 = document.getElementById("a2");
const a3 = document.getElementById("a3");

a1.addEventListener("click", () => {
    a1.style.backgroundColor = "red";
});

a2.addEventListener("click", () => {
    a2.style.backgroundColor = "green";
});

a3.addEventListener("click", () => {
    a3.style.backgroundColor = "blue";
});
```

## getElementsByTagName

```html
<div id="a1">HTML</div>
<div id="a2">CSS</div>
<div id="a3">JAVASCRIPT</div>
```

```javascript
const b1 = [...document.getElementsByTagName("div")];

b1.forEach(element => {
    element.addEventListener("click", () => {
        element.style.backgroundColor = "red";
    });
});
```

## getElementsByClassName

```html
<div id="a1" class="b1">HTML</div>
<div id="a2" class="b1">CSS</div>
<div id="a3" class="b2">JAVASCRIPT</div>
```

```javascript
const b1 = [...document.getElementsByClassName("b1")];
const b2 = [...document.getElementsByClassName("b2")];

b1.forEach(element => {
    element.addEventListener("click", () => {
        element.style.backgroundColor = "red";
    });
});

b2.forEach(element => {
    element.addEventListener("click", () => {
        element.style.backgroundColor = "blue";
    });
});
```
