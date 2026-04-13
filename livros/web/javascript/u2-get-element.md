# Sumário

- [getElementById](#getelementbyid)
- [getElementsByTagName](#getelementsbytagname)
- [getElementsByClassName](#getelementsbyclassname)
- [querySelector](#querySelector)
- [querySelectorAll](#querySelectorAll)

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

b1.forEach((element) => {
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

b1.forEach((element) => {
  element.addEventListener("click", () => {
    element.style.backgroundColor = "red";
  });
});

b2.forEach((element) => {
  element.addEventListener("click", () => {
    element.style.backgroundColor = "blue";
  });
});
```

## querySelector

```md
// div
document.querySelector("div")

// .curso - classe
document.querySelector(".curso")

// #curso - id
document.querySelector("#curso")

// div.curso - div com classe curso
document.querySelector("div.curso")

// div#curso - div com id curso
document.querySelector("div#curso")

// div#curso.curso - div com id curso e classe curso
document.querySelector("div#curso.curso")

// div#curso.curso.outro - div com id curso e classe curso e outra classe
document.querySelector("div#curso.curso.outro")

// div > p - p filho direto de div
document.querySelector("div > p")

// div p - p descendente de div (qualquer nível)
document.querySelector("div p")
```

```html
<div id="a1" class="b1">HTML</div>
<div id="a2" class="b1">CSS</div>
<div id="a3" class="b2">JAVASCRIPT</div>
```

```javascript
// seleciona o primeiro elemento apenas
const query_selector = document.querySelector("div");

query_selector.addEventListener("click", () => {
  query_selector.style.backgroundColor = "red";
});
```

## querySelectorAll

```html
<div id="a1" class="b1">HTML</div>
<div id="a2" class="b1">CSS</div>
<div id="a3" class="b2">JAVASCRIPT</div>
```

```javascript
const query_selector_all = [...document.querySelectorAll("div")];

query_selector_all.forEach((element) => {
  element.addEventListener("click", () => {
    element.style.backgroundColor = "red";
  });
});
```
