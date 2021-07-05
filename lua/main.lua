
function add(a, b)
   return a+b
end


function update_db()
   a = list_channels()
   for i,v in pairs(a) do
      r_print(v)
   end
end
