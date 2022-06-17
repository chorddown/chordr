/**
 * @param {boolean} enable
 * @param {string} module
 * @returns Console
 */
function buildOutput(enable, module) {
    if (typeof module !== 'string') {
        module = ''
    }
    const consoleStyles = {
        normalStyle: "background: inherit; color: inherit",
        pathStyle: "font-weight: bold; color: inherit",
        label: {
            info: {
                text: "%cINFO%c webchordr " + module + "%c ",
                style: "color: white; padding: 0 3px; background: #029202;"
            },
            error: {
                text: "%cERROR%c webchordr " + module + "%c ",
                style: "color: white; padding: 0 3px; background: #ff2863;"
            },
            warn: {
                text: "%cWARN%c webchordr " + module + "%c ",
                style: "color: white; padding: 0 3px; background: #c18d12;"
            },
            debug: {
                text: "%cDEBUG%c webchordr " + module + "%c ",
                style: "color: white; padding: 0 3px; background: #0066ff;"
            },
        }
    }

    if (!enable) {
        const ef = () => {
        };

        return {debug: ef, info: ef, warn: ef, error: ef};
    }

    const root = typeof window === 'object' ? window : self;
    return {
        debug: root.console.debug.bind(root.console, consoleStyles.label.debug.text + '%s', consoleStyles.label.debug.style, consoleStyles.pathStyle, consoleStyles.normalStyle),
        info: root.console.info.bind(root.console, consoleStyles.label.info.text + '%s', consoleStyles.label.info.style, consoleStyles.pathStyle, consoleStyles.normalStyle),
        warn: root.console.warn.bind(root.console, consoleStyles.label.warn.text + '%s', consoleStyles.label.warn.style, consoleStyles.pathStyle, consoleStyles.normalStyle),
        error: root.console.error.bind(root.console, consoleStyles.label.error.text + '%s', consoleStyles.label.error.style, consoleStyles.pathStyle, consoleStyles.normalStyle),
    }
}
