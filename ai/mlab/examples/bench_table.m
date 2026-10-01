% In-memory table benchmark: 1 million rows, grouping, sum, sorting (mlab run examples/bench_table.m)
n = 1e6;
rand('seed', 1);
g = randi(1000, n, 1); r = randi(12, n, 1); x = rand(n, 1);
tic; T = table(g, r, x); t0 = toc;
tic; G = groupsummary(T, 'g', 'sum', 'x'); G = sortrows(G, 'sum_x', 'descend'); t1 = toc;
tic; S = sortrows(T, 'x'); t2 = toc;
tic; G2 = groupsummary(T, {'g','r'}, {'sum','mean'}, 'x'); t3 = toc;
tic; T.y = T.x .* 2 + T.g; T.big = T.x > 0.5; t4 = toc;
tic; U = grouptransform(T, 'g', @cumsum, 'x'); t5 = toc;
tic; F = T(T.big & T.r == 3, :); t6 = toc;
printf('table (3 columns, %d rows)                    %.3f s\n', n, t0);
printf('groupsummary sum by g (1000 groups) + sortrows %.3f s\n', t1);
printf('sortrows of the whole table by x               %.3f s\n', t2);
printf('groupsummary sum+mean by (g, r) (%d groups) %.3f s\n', height(G2), t3);
printf('two computed columns                           %.3f s\n', t4);
printf('grouptransform cumsum by g                     %.3f s\n', t5);
printf('mask filter (%d rows)                      %.3f s\n', height(F), t6);
printf('check: sum of sum_x = sum of x: %d\n', abs(sum(G.sum_x) - sum(x)) < 1e-6 * n);
