class Screen {
    
    constructor(id, domNode, onShow, onHide) {
        this.id = id;

        this.domNode = domNode;
        this.defaultDisplay = domNode.style.display;

        this.onShow = onShow;
        this.onHide = onHide;

        this.innerHtml = new Proxy({}, {
            get() {return domNode.innerHtml},

            set(value) {domNode.innerHtml = value}
        });

        domNode.style.display = "none";
    }

    static fromElementId(id, onShow, onHide) {
        return new Screen(id, document.getElementById(id), onShow, onHide);
    }

    querySelector(selector) {
        return this.domNode.querySelector(selector);
    }

    appendChild(child) {
        this.domNode.appendChild(child);
    }

    setAttribute(attribute, value) {
        this.domNode.setAttribute(attribute, value);
    }

    removeAttribute(attribute) {
        this.domNode.removeAttribute(attribute);
    }

    show(...args) {
        this.domNode.style.display = this.defaultDisplay;
        
        if (this.onShow) this.onShow(...args);
    }

    hide() {
        if (this.onHide && this.domNode.style.display != "none") this.onHide();
            
        this.domNode.style.display = "none";
    }
}

class ScreenSwitch {

    constructor() {
        this.screens = [];

        this.currentScreen = null;
    }

    withScreen(screen) {
        if (!screen instanceof Screen) throw new TypeError(`Expected Screen, found ${typeof screen}`);

        this.screens.push(screen);

        if (this.currentScreen == null) this.currentScreen = screen.id;

        return this;
    }

    withDefaultScreen(screen) {
        if (!screen instanceof Screen) throw new TypeError(`Expected Screen, found ${typeof screen}`);

        this.screens.push(screen);

        this.currentScreen = screen.id;

        return this;
    }

    showScreen(id, ...args) {
        for (const screen of this.screens) {
            if (screen.id == id) screen.show(...args);
            else screen.hide();
        }

        window.history.pushState({}, "", window.location);
    }

}