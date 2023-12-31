#include <stdio.h>
#include <time.h>

struct State {
    float position;
    float velocity;
};

struct Derivative {
    float position_derivative;
    float velocity_derivative;
};

Derivative evaluate(const State & initial, double t, float dt, const Derivative & d ) {
    
    State state;
        state.position = initial.position + d.position_derivative*dt;
        state.velocity = initial.velocity + d.velocity_derivative*dt;

    Derivative output;
        output.position_derivative = state.velocity;
        output.velocity_derivative = acceleration( state, t+dt );
    
    return output;
}

float acceleration(const State & state, double t){
    const float k = 15.0f;
    const float b = 0.1f;
    return -k*state.position - b * state.velocity;
}

void integrate(State & state, double t, float dt) {
    Derivative a,b,c,d;

    a = evaluate(state, t, 0.0f, Derivative());
    b = evaluate(state, t, dt*0.5f, a);
    c = evaluate(state, t, dt*0.5f, b);
    d = evaluate(state, t, dt, c);

    float dxdt = 1.0f /6.0f * (a.position_derivative + 2.0f * (b.position_derivative + c.position_derivative) + d.position_derivative);
    float dvdt = 1.0f /6.0f * (a.velocity_derivative + 2.0f * (b.velocity_derivative + c.velocity_derivative) + d.velocity_derivative);

    state.position = state.position + dxdt * dt;
    state.velocity = state.velocity + dvdt * dt;
}

void semi_implicit_euler() {
    
    double t = 0.0;
    float dt = 1.0f;

    float velocity = 0.0f;
    float position = 0.0f;
    float force = 10.0f;
    float mass = 1.0f;

    while ( t <= 10.0 )
    {
        position = position + velocity * dt;
        velocity = velocity + ( force / mass ) * dt;
        t += dt;
    }
}


int main() {

    //Let’s call our current time t, and the time step dt or ‘delta time’.
    double t = 0.0;
    double dt = 0.01;
    bool quit = false;

    double currentTime = current_time_in_seconds();
    double accumulator = 0.0;

    State previousState;
    State currentState;

    while ( !quit )
    {
        double newTime = current_time_in_seconds();
        double frameTime = newTime - currentTime;
        if ( frameTime > 0.25 )
            frameTime = 0.25;
        currentTime = newTime;

        accumulator += frameTime;

        while ( accumulator >= dt )
        {
            previousState = currentState;
            integrate( currentState, t, dt );
            t += dt;
            accumulator -= dt;
        }

        const double alpha = accumulator / dt;

        /*State state = currentState * alpha + 
            previousState * ( 1.0 - alpha );*/

        //render( state );
    }

    return 0;
}

float current_time_in_seconds(){
    time_t seconds;
    seconds = time(NULL);
    return seconds;
}