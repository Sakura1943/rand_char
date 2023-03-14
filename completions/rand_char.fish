complete -c rand_char -s l -l length -d 'The length of the generated single string' -r
complete -c rand_char -s c -l count -d 'Number of strings generated' -r
complete -c rand_char -s s -l save -d 'The path where the result will be saved' -r -F
complete -c rand_char -s i -l ignore -d 'Ignore dangerous words'
complete -c rand_char -l ignore-symbol -d 'Ignore Symbol'
complete -c rand_char -l only-number -d 'Only Number(integer)'
complete -c rand_char -l only-letter -d 'Only letter'
complete -c rand_char -l only-uppercase -d 'Only letters in upper case'
complete -c rand_char -l only-lowercase -d 'Only letters in lower case'
complete -c rand_char -s h -l help -d 'Print help'
complete -c rand_char -s V -l version -d 'Print version'
