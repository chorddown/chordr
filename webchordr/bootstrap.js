// Styles could be bundled with the built JS
// import './static/stylesheets/chordr-default-styles.scss';
// import './static/stylesheets/chordr-app.scss';
import ClipboardJS from 'clipboard'
import {SortableWrapper} from './static/assets/javascripts/SortableWrapper'

// window.SortableWrapper = SortableWrapper;

new ClipboardJS('[data-clipboard-target]');

import("./pkg").then(module => {
    module.run_app();
});
