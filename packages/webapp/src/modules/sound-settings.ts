import { GemElement, html, adoptedStyle, customElement, connectStore } from '@mantou/gem';
import { throttle } from 'duoyun-ui/lib/utils';

import { configure, Settings } from 'src/configure';
import { i18n } from 'src/i18n';
import { gridStyle } from 'src/modules/shortcut-settings';
import { updateAccount } from 'src/services/api';
import { playSound } from 'src/utils';

import 'duoyun-ui/elements/slider';

/**
 * @customElement m-sound-settings
 */
@customElement('m-sound-settings')
@adoptedStyle(gridStyle)
@connectStore(i18n.store)
export class MSoundSettingsElement extends GemElement {
  #throttleUpdateVolume = throttle(async (name: string, value: number) => {
    await updateAccount({
      settings: {
        ...configure.user!.settings,
        volume: {
          ...configure.user!.settings.volume,
          [name]: value,
        },
      },
    });
    playSound('', value);
  });

  render = () => {
    if (!configure.user) return html``;

    const volumeLabelMap: Record<keyof Settings['volume'], string> = {
      hint: i18n.get('hintVolume'),
      notification: i18n.get('notificationVolume'),
      game: i18n.get('gameVolume'),
    };

    return html`
      <dy-heading class="heading" lv="4">${i18n.get('volumeSetting')}</dy-heading>
      <div class="grid">
        ${Object.entries(configure.user.settings.volume).map(
          ([name, value]) => html`
            <div>${volumeLabelMap[name as keyof Settings['volume']]}</div>
            <dy-slider
              .value=${value * 100}
              @change=${(evt: CustomEvent<number>) => {
                if (Math.round(value * 100) !== evt.detail) {
                  (evt.target as any).value = evt.detail;
                  this.#throttleUpdateVolume(name, evt.detail / 100);
                }
              }}
            ></dy-slider>
          `,
        )}
      </div>
    `;
  };
}
