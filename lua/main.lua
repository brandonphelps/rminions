
function add(a, b)
   return a+b
end


function update_db()
   a = list_channels()
   for i,v in pairs(a) do
      r_print(v)
   end
end

my_cool_state = {}
my_cool_state.time = 0

function update_me(time_dt)

   my_cool_state.time = my_cool_state.time + time_dt
   return tostring(my_cool_state.time)
end

-- must require lua folder,
-- i.e is from the root of the binary.
dofile('lua/other.lua')




