use teremder::*;

fn main() {
    set_target_fps(30.);
    set_exit_key_combo([Key::LeftControl, Key::C]);

    loop {
        clear_background(BLACK);

        let mut x = 2.;
        let mut y = 1.;

        let mut key = |k: Key, w: u16, nl: bool| {
            fill_rect(x, y, w as f32, 2., if is_key_down(k) { RED } else { GRAY });
            x += w as f32 + 1.;
            if nl {
                x = 2.;
                y += 3.;
            }
        };

        {
            use Key::*;
            key(Num1, 2, false);
            key(Num2, 2, false);
            key(Num3, 2, false);
            key(Num4, 2, false);
            key(Num5, 2, false);
            key(Num6, 2, false);
            key(Num7, 2, false);
            key(Num8, 2, false);
            key(Num9, 2, false);
            key(Num0, 2, true);

            key(Q, 2, false);
            key(W, 2, false);
            key(E, 2, false);
            key(R, 2, false);
            key(T, 2, false);
            key(Y, 2, false);
            key(U, 2, false);
            key(I, 2, false);
            key(O, 2, false);
            key(P, 2, true);

            key(A, 2, false);
            key(S, 2, false);
            key(D, 2, false);
            key(F, 2, false);
            key(G, 2, false);
            key(H, 2, false);
            key(J, 2, false);
            key(K, 2, false);
            key(L, 2, true);

            key(Z, 2, false);
            key(X, 2, false);
            key(C, 2, false);
            key(V, 2, false);
            key(B, 2, false);
            key(N, 2, false);
            key(M, 2, true);

            key(LeftShift, 2, false);
            key(LeftControl, 2, false);
            key(Space, 2, false);
            key(RightControl, 2, false);
            key(RightShift, 2, false);
        }

        next_frame();
    }
}
