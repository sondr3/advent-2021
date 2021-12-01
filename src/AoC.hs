module AoC (runDay) where

runDay :: Int -> IO ()
runDay day = case day of
  1 -> putStrLn "1"
  _ -> putStrLn "Missing day"
