# Web Design Responsivo - Páginas adaptáveis para todos os dispositivos

O web design resposivo, nada mais é que: wed design que responder a quaisquer dispositivos/resoluções e, devido a uma série de caracteristicas técnicas bem específicas, é bem apresentado em qualquer um deles.

## O que tem que possuir?

- Layout fluído
- Imagens e recursos flexíveis;
- Media queries.

O layout deve ser construído de forma a se adaptar a diferentes resuluções de tela, tamanhos mesmo que diferentes possam permitir que o conteúdo possa ser visualizado de forma perfeita.

Os assets, como por exemplo: imagens, vídeos, etc; devem se adaptar as transformações do layout, um menu não preciso aparecer por completo se assim não for necessário.

Por fim a media queries, que garante que nem todos elementos tenham que aparecer na tela sem necessidade, principalmente em aparelhos menores em que nem todas as sessões ficam visivéis.

## Layout fluído

O layout fluído ou grid flexivel, não deve usar medida absolutas no CSS.

### Medidas do CSS

- Pixel (px);
- Ponto (point);
- Ems (em);
- Porcentagem(%).

Os valores em "ems" e "porcetagens", são relativos, escaláveis e se adaptam, mantendo relações de tamanho com outros elementos de um documento.

O “consenso” do mercado recomenda usar porcentagem para lidar com tamanhos no layout (larguras, margens, espaçamentos, etc) e usar ems para lidar com fontes.

```css
.container {
margin: 0 auto;
width: 67.5%; /* +/- 960 */
}

/* Antes */
h1 {
font-size: 32px;
}

/* Depois */
h1 {
font-size: 2em; /* 32 / 16 */
}
```

### Metatag viewport

A algo importante antes de começar a escrever todo o código da página, é ajustar no HTML a configuração referente a meta tag viewport, pois ela permite que todo o site seja desenvolvido tomando como base o tamanho ideial de cada dispositivo.

A configuração recomendada da viewport:

```html
<meta name="viewport" content="width=device-width,initial-scale=1">
```

Algumas outras configurações podem ser feitas, mas leve em consideração o seu projeto. Como por exemplo:

```html
<meta name="viewport" content="width=device-width,initial-scale=1,maximum-scale=1">
```

A `maximum-scale=1` impede que o usuário da página possa aplicar um zoom.



