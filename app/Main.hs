module Main where

{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE RecordWildCards   #-}

-- Filesystem path utils
import qualified Turtle
import Prelude hiding (FilePath)
import Filesystem.Path.CurrentOS as Path

-- standard libraries
import Options.Applicative
import Control.Monad
import Data.SemiGroup ((<>))

-- String-like utils
import qualified Data.Text as T
import qualified Data.Text.Encoding as T.Encoding
import qualified Data.ByteString.Lazy as B

-- Used for colouring our console outputs
import qualified System.Console.ANSI as ANSI

-- Our actual program library.
import Lib

-- The data structure describing available commands
data Command = CommandList |

    CommandCommit {
    } |

    CommandConfig {
    }

  deriving (Show)

showHelpOnErrorExecParser :: ParserInfo a -> IO a
showHelpOnErrorExecParser = customExecParser (prefs showHelpOnError)

main :: IO()
main = do

  -- command :: Command
  command <- showHelpOnErrorExecParser (info (helper <*> parseCommand)(
      fullDesc <>
      progDesc "Gin is a client running on libgit2 that provides a more corporate-friendly " ++
               "workflow for git, and avoid unnecessary database duplication over multiple " ++
               "workspaces." <>
      header   "Gin: Version control for those who could really use a drink." ))

  -- run :: IO ()
  run command

parseCommand :: Parser Command
parseCommand = subparser $

  (command "commit"
    (info
      (helper <*> parseCommitCommand)
      (fullDesc <> progDesc "Manipulate the current working commit.")))

   <>

  (command "config"
    (info
      (helper <*> parseConfigCommand)
      (fullDesc <> progDesc "View or edit the current config.")))

parseCommitCommand :: Parser Command
parseCommitCommand = pure (CommandCommit)

parseConfigCommand :: Parser Command
parseConfigCommand = pure (CommandConfig)
