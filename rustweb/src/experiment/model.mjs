const FPS = 30;
const TIME = 5 * 60 * FPS;
const MAP_W = 32, MAP_H = 32;

// let clone = Object.assign( Object.create( Object.getPrototypeOf(orig)), orig)

class World {
    constructor() {
        this.frames = Array(TIME);
    }
}

class ObservedState {
    constructor() {
        
    }

    matches(locations, other) {
        // (other ⊂ locations) == (this ⊂ locations)

    }
}

class Action {
    constructor(opt) {
        /*
        An observable action will apply even if the location affected
        is observed. If the action isn't magical, the action must be replicated by you
        before you can advance.
        An unobservable action will cancel if observed and will never happen after that.
        */
        this.observable = opt.observable;

        /*
        A magical action will not have to be fulfilled by you.
        */
        this.magical = opt.magical || false;

        this.replicated = opt.replicated || false;
    }

    shouldPerform(observableLocations, affectedLocations) {
        return this.observable || !observableLocations.contains(affectedLocations);
    }
}

class MoveAction extends Action {
    constructor(what, from, to) {
        super({ observable: false, magical: false, replicated: false });
        this.what = what;
        this.from = from;
        this.to = to;
    }

    perform(state, observableLocations) {
        if (this.shouldPerform(observableLocations, [from, to])) {
            if (state.hasInLocation(this.what, this.from)) {
                state.remove(this.from);
                state.set(this.from, this.what);
            }
        }
    }
}

class DoorAction extends Action {
    constructor(where, state, replicated) {
        super({ observable: true, replicated });
        this.where = where;
        this.state = state;
    }

    perform(state, observableLocations) {
        state.setDoorState(this.where, this.state);
    }
}

class State {
    constructor() {
        this.map = Array(MAP_W * MAP_H);
    }

    process() {

    }
}