% fib(27) by recursion: user function calls, compare and jump, scalar arithmetic
disp(fib(27))

function r = fib(n)
  if n < 2
    r = n;
  else
    r = fib(n-1) + fib(n-2);
  end
end
