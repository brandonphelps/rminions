
-- updates db with rust functions.
function update_db()
   a = list_channels()
   for i,v in pairs(a) do
      populate_db(v)
   end
end


function man_vid(video)
   return video.title
end
