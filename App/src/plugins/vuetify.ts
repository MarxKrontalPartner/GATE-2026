import { createVuetify, type ThemeDefinition } from 'vuetify';
import 'vuetify/styles';
import * as components from 'vuetify/components';
import * as directives from 'vuetify/directives';
import { mdi, aliases } from 'vuetify/iconsets/mdi-svg';


const mkpLightBlue = '#42c0eb';
const mkpMediumBlue = '#12a5e2';
const mkpDarkBlue = '#004972';
const mkpLightenGrey1 = '#BDBDBD';
const mkpDarkGrey = '#424242';
const mkpError = '#B00020';
const mkpWarning = '#EAB308';
const mkpSucces = '#84cc16';
const white = '#FFFFFF';

const mkpLightTheme: ThemeDefinition = {
  dark: false,
  colors: {
    background: '#FFFFFF',
    surface: '#F8F9FA',
    'surface-darken-1': "#EEEEEE",
    primary: mkpLightBlue,
    'primary-darken-1': mkpMediumBlue,
    'on-primary': white, // text on primary should be bright, because primary is relatively dark
    'on-surface': mkpDarkBlue,  // changes text color on something like cards. But also influences overlay color.
    secondary: mkpDarkBlue,
    'secondary-darken-1': '#018786',
    error: mkpError,
    info: '#2196F3',
    success: mkpSucces,
    warning: mkpWarning,
    text: mkpDarkBlue,
    border: mkpLightenGrey1,
    'code-background': '#eeeeee',
    icon: "#718FA8",
    'grid-readonly-cell-shadow-color': '#000000',
  },
  variables: {
    'grid-readonly-cell-border-opacity': 1,
    'overlay-opacity': 0.18,
  },
}

const mkpDarkTheme: ThemeDefinition = {
    dark: true,
    colors: {
      background: '#333333',
      surface: '#555658',
      'surface-darken-1': "#4c4c4c",
      primary: mkpLightBlue,
      'primary-darken-1': mkpMediumBlue,
      'on-primary': white, // text on primary should be bright, because primary is relatively dark
      'on-surface': white,  // changes text color on something like cards. But also influences overlay color.
      secondary: mkpLightBlue,
      'secondary-darken-1': mkpLightBlue,
      error: '#FF7575',
      info: '#2196F3',
      success: mkpSucces,
      warning: mkpWarning,
      text: '#FFFFFF',
      border: mkpDarkGrey,
      'code-background': '#212121',
      icon: "#BDBDBD",
      'grid-readonly-cell-shadow-color': '#FFFFFF',
  },
  variables: {
    'grid-readonly-cell-border-opacity': 0,
    'overlay-opacity': 0.5,
  },
}


const vuetify = createVuetify({
    components,
    directives,
    icons: {
        defaultSet: 'mdi',
        aliases,
        sets: {
          mdi
        },
    },
    theme: {
        defaultTheme: 'mkpLightTheme',
        themes: {
          mkpLightTheme,
          mkpDarkTheme
        },
        
    },
})

export default vuetify;