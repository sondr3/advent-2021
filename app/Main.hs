module Main (main) where

import qualified AoC (runDay)
import Options.Applicative

newtype App = App {day :: Int}

app :: Parser App
app =
  App
    <$> option
      auto
      ( long "day"
          <> short 'd'
          <> metavar "INT"
          <> help "Day to run"
      )

run :: App -> IO ()
run (App day) = do
  putStrLn "Getting solution..."
  AoC.runDay day

main :: IO ()
main = run =<< execParser opts
  where
    opts =
      info
        (app <**> helper)
        (fullDesc <> progDesc "Run an AoC day" <> header "aoc - an AoC thing")
