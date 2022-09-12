import { GemElement, html, adoptedStyle, customElement, createCSSSheet, css } from '@mantou/gem';

import { theme } from 'src/theme';

const style = createCSSSheet(css`
  :host {
    display: block;
    padding-inline: ${theme.gridGutter};
  }
  footer {
    padding: ${theme.gridGutter};
    text-align: center;
  }
`);

/**
 * @customElement m-footer
 */
@customElement('m-footer')
@adoptedStyle(style)
export class MFooterElement extends GemElement {
  render = () => {
    return html`<footer>NESBox © Copyright</footer>`;
  };
}
