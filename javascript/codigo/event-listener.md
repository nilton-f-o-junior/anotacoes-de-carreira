# Sumário

- [getElementById](#getelementbyid)

## EventListener

```html
<div id="a1" class="b1">HTML</div>
<div id="a2" class="b1">CSS</div>
<div id="a3" class="b2">JAVASCRIPT</div>
```

```css
.destaque {
  background-color: burlywood;
}
```

```javascript
const b1 = [...document.querySelectorAll(".b1")];

b1.forEach((el) => {
  el.addEventListener("click", () => {
    el.classList.add("destaque");
  });
});
```
