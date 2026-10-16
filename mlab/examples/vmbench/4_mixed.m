% mixed scenario: matrix builtins (\, *, norm, sum, abs) in a loop + scalar builtins (mod, sqrt, abs)
rand('seed', 26);
A = rand(200) + 200 * eye(200);
b = rand(200, 1);
s = 0;
for k = 1:40
  x = A \ b;
  y = A * x;
  s = s + norm(y - b) + sum(abs(x)) / numel(x);
  b = b + 0.001 * y;
end
t = 0;
for k = 1:200000
  t = t + mod(k, 7) * sqrt(abs(k - 5)) / k;
end
printf('%.8f %.8f\n', s, t)
