@echo off
setlocal enabledelayedexpansion

set "file_list="

:loop
if "%~1"=="" goto execute_command
set "file_list=!file_list! "%~1""
shift
goto loop

:execute_command
parallel_executor %file_list%