
function factorial(n)
   if n == 0 then
      return 1
   elseif n == 1 then
      return 1
   else
      return factorial(n-1) * n
   end
end


