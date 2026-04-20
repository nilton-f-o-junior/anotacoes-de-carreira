# Sumário

```html
<div id="caixa_a" class="caixa">
  <div id="a1" class="curso b1">HTML</div>
  <div id="a2" class="curso b1">CSS</div>
  <div id="a3" class="curso b1">JAVASCRIPT</div>
</div>

<button id="btn_copiar">Copiar</button>
<div id="caixa_b" class="caixa"></div>
```

```css
* {
    padding: 0px;
    margin: 0%;
    border: none;
}

main {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
}

button {
    width: 150px;
    height: 40px;
    background-color: #000;
    color: #fff;
    cursor: pointer;
    border-radius: 10px;
}

.curso {
    display: flex;
    justify-content: center;
    width: 200px;
    border: 4px solid #000;
    border-radius: 10px;
    padding: 10px;
    margin: 5px 0px;
    cursor: pointer;
}

.b1 {
    background-color: #ccc;
    color: #444;
}

.b2 {
    background-color: #444;
    color: #ccc;
}

.caixa {
    border: 4px solid #000;
    background-color: #eee;
    padding: 10px;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    margin: 5px;
    height: 500px;
    width: 300px;
}

.selecionar {
    background-color: rgb(134, 74, 194);
    color: rgb(255, 255, 255);
    border-color: rgb(243, 205, 205);
}
```

## add

```javascript
// seleciona e não desseleciona
const caixa_a1 = document.querySelector("#a1");
const caixa_a2 = document.querySelector("#a2");
const btn = document.querySelector("#btn_copiar");
const tdCursos = [...document.querySelectorAll(".curso")];

tdCursos.map((el) => {
    el.addEventListener("click", (evt) => {
        const curso = evt.target;
        curso.classList.add("selecionar");
    })
})
```

## toggle

```javascript
// alterna entre seleciona e desseleciona
const caixa_a1 = document.querySelector("#a1");
const caixa_a2 = document.querySelector("#a2");
const btn = document.querySelector("#btn_copiar");
const tdCursos = [...document.querySelectorAll(".curso")];

tdCursos.map((el) => {
    el.addEventListener("click", (evt) => {
        const curso = evt.target;
        curso.classList.add("selecionar");
    })
})
```