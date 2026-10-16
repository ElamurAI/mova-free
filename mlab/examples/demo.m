% mlab: a short demo. Run: mlab run examples/demo.m
% Gate negative control: mlab run examples/demo.m --fault gate:mldivide

% matrices, indexing, end, logical masks
A = magic(4);
disp(A(2:end, [1 end]))
big = A(A > 12)'

% linear system with a check (the gate stays silent if the answer is correct)
M = [4 -2 1; -2 4 -2; 1 -2 4];
b = [11; -16; 17];
x = M \ b
residual = norm(M * x - b)

% eigenvalues and eigenvectors: A·V = V·D
[V, D] = eig([2 1; 1 2]);
disp(norm([2 1; 1 2] * V - V * D) < 1e-12)

% polynomials and roots
r = roots([1 -6 11 -6])'

% a user function and an anonymous one
f = @(t) exp(-t) .* cos(2 * pi * t);
q = integral(f, 0, Inf)
fprintf('fib(20) = %d\n', fib(20));

% ODE: oscillator
[t, y] = ode45(@(t, y) [y(2); -y(1)], [0 pi/2 pi], [1; 0]);
fprintf('y(pi) = [%.4f %.4f]\n', y(end, 1), y(end, 2));

function r = fib(n)
  a = 0; b = 1;
  for k = 1:n
    [a, b] = deal2(b, a + b);
  end
  r = a;
end

function [x, y] = deal2(p, q)
  x = p;
  y = q;
end
