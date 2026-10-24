// Suitable for Fresh 0.5.4
//
// cSpell: enabled

// ? theme
//
// Fresh's `theme` config field holds a single value, so follow the system
// color scheme here instead, mirroring VS Code's `window.autoDetectColorScheme`.
//
// Top-level `await` does not work here: init.ts is loaded as a plugin and
// evaluated synchronously, so awaiting at the top level is a SyntaxError that
// aborts this whole file. `fresh --cmd init check` does not catch it — it wraps
// the source in an async body, which the real loader does not. Hence the async
// IIFE. It must swallow its own errors, because an unhandled rejection takes
// the entire plugin runtime down with it.
void (async () => {
  try {
    const colorScheme = await editor.spawnProcess('gsettings', [
      'get',
      'org.gnome.desktop.interface',
      'color-scheme',
    ]);

    editor.applyTheme(
      colorScheme.exit_code === 0 && colorScheme.stdout.includes('dark')
        ? 'terminal-dark'
        : 'terminal-light',
    );
  } catch (error) {
    editor.applyTheme('terminal-light');
  }
})();
