// https://github.com/tweakpane/plugin-essentials/issues/8#issuecomment-1084332974
export function addFilePicker(pane, title, callback) {
    return pane.addButton({
        title,
    }).on('click', () => {
        const input = document.createElement('input');
        input.setAttribute('type', 'file');
        input.style.opacity = '0';
        input.style.position = 'fixed';
        document.body.appendChild(input);
        input.addEventListener('input', (e) => {
            const file = input.files[0];
            document.body.removeChild(input);
            callback(file);
        }, { once: true })
        input.click();
    })
}

export function addJsonFilePicker(pane, title, callback) {
    return addFilePicker(pane, title, f => {
        const reader = new FileReader();
        reader.onload = e => callback(JSON.parse(e.target.result));
        reader.readAsText(f);
    });
}
