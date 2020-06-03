// Styles could be bundled with the built JS
// import './static/stylesheets/chordr-default-styles.scss';
// import './static/stylesheets/chordr-app.scss';
import ClipboardJS from 'clipboard'
import './src-typescript/SortableWrapper'

new ClipboardJS('[data-clipboard-target]');

// @ts-ignore
import("./pkg").then(module => {
    module.run_app();
});
