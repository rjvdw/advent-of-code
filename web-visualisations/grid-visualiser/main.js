import { render, html } from 'lit-html'

const DOM = {
  get textarea() {
    return document.querySelector('#input-form textarea[name="input"]')
  },

  get gridContainer() {
    return document.getElementById('grid-container')
  },
}

updateGrid()

DOM.textarea.addEventListener('input', (event) => {
  const input = event.target.value
  const lines = input.split('\n')

  const columns = lines
    .map((line) => line.length)
    .reduce((acc, v) => Math.max(acc, v), 25)
  const rows = Math.max(lines.length, 25)

  event.target.style.inlineSize = `${columns}ch`
  event.target.style.blockSize = `${rows}lh`

  updateGrid()
})

function updateGrid() {
  const input = DOM.textarea.value
  const container = DOM.gridContainer

  if (!input) {
    container.hidden = true
    return
  }

  container.hidden = false

  render(
    html`
      <div class="input-grid">
        ${input
          .split('\n')
          .map(
            (line) => html`
              <div class="input-row">
                ${line
                  .split('')
                  .map(
                    (ch) => html`
                      <span class="${cellClassName(ch)}">${ch}</span>
                    `,
                  )}
              </div>
            `,
          )}
      </div>
    `,
    container,
  )
}

function cellClassName(ch) {
  const parts = ['input-cell']
  switch (ch) {
    case ' ':
    case '#':
      parts.push('blocked')
      break
    case '.':
      parts.push('empty')
  }
  return parts.join(' ')
}
