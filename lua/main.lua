
function add(a, b)
   return a+b
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
dofile('lua/vidlid.lua')



