% nested loops over a 1000×1000 matrix with elementwise work
n = 1000;
rand('seed', 26);
A = rand(n);
B = zeros(n);
for j = 1:n
  for i = 1:n
    x = A(i, j);
    B(i, j) = x * x + 2 * x + 1;
  end
end
printf('%.10f\n', sum(B(:)) / numel(B))
