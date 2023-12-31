use std::time::SystemTime;


//Semi-implicit Euler (Cheap)
fn _semi_implicit_euler() {
    let mut t = 0.0;
    let dt = 1.0f64;

    let mut velocity = 0.0f64;
    let mut position = 0.0f64;
    let force = 10.0f64;
    let mass = 1.0f64;

    while(t <= 10.0) {
        velocity = velocity + (force/mass) * dt;
        position = position + velocity *dt;
        t += dt;
    }
}

//RK4 integrator (Expensive)
#[derive(Clone, Copy)]
pub struct State{
    pub position: f64,
    pub velocity: f64,
}

impl State{
    pub fn new() -> State {
        let x = State {
            position: 0.0f64,
            velocity: 0.0f64,
        };
        return x
    }
}

#[derive(Clone, Copy)]
pub struct Derivative{
    pub position_derivative: f64,
    pub velocity_derivative: f64,
}

impl Derivative{
    fn new() -> Derivative {
        let x = Derivative {
            position_derivative: 0.0f64,
            velocity_derivative: 0.0f64,
        };
        return x
    }
}


pub fn evaluate(initial: State, mut time: f64, mut delta_time: f64, d: Derivative) -> Derivative {
    let mut state = State{
        position: initial.position + d.position_derivative*delta_time,
        velocity: initial.velocity + d.velocity_derivative*delta_time,
    };
    let mut output = Derivative {
        position_derivative: state.velocity,
        velocity_derivative: acceleration(state, time+delta_time),
    };
    return output
}

pub fn acceleration(state: State, mut time: f64) -> f64 {
    let k: f64 = 15.0f64;
    let b: f64 = 0.1f64;
    return -k*state.position - b * state.velocity; 
}

pub fn integrate(mut state: State, mut time: f64, mut delta_time: f64 ) {
    
    let a = evaluate(state, time, 0.0f64, Derivative::new());
    let b = evaluate(state, time, delta_time*0.5f64, a);
    let c = evaluate(state, time, delta_time*0.5f64, b);
    let d = evaluate(state, time, delta_time, c);
    
    let derivative_position_delta_time: f64 = 1.0f64 / 6.0f64 * (a.position_derivative + 2.0f64 * (b.position_derivative + c.position_derivative) + d.position_derivative);
    let derivative_velocity_delta_time: f64 = 1.0f64 / 6.0f64 * (a.velocity_derivative + 2.0f64 * (b.velocity_derivative + c.velocity_derivative) + d.velocity_derivative);

    state.position = state.position + derivative_position_delta_time * delta_time;
    state.velocity = state.velocity + derivative_velocity_delta_time * delta_time;
}

fn timestep() {
    let mut time : f64 = 0.0;
    let delta_time: f64 = 0.01;
    let quit = false;

    let mut current_time = SystemTime::now();
    let mut accumulator: f64 = 0.0;

    let mut previous_state = State::new();
    let current_state = State::new();

    while !quit {
        let new_time = SystemTime::now();
        let mut frame_time = new_time.duration_since(current_time).unwrap().as_secs_f64();
        if frame_time > 0.25 {
            frame_time = 0.25;
        }
        current_time = new_time;

        accumulator += frame_time;

        while accumulator >= delta_time {
            previous_state = current_state.clone();
            integrate(current_state, time, delta_time);
            time += delta_time;
            accumulator -= delta_time;
        }
        let alpha = accumulator / delta_time;
        //let state = currentState * alpha + previousState * (1.0 - alpha);

    }
    //render(state);

}

pub fn interpolate(mut state: State, mut previousState: State, mut alpha: f64) -> State {
    state.position = state.position * alpha + previousState.position * (1.0 - alpha);
    state.velocity = state.velocity * alpha + previousState.velocity * (1.0 - alpha);
    return state
}