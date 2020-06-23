if [ -f $HOME/.zshrc ]; then
  echo export PATH=\$PATH:`pwd`/bin >> $HOME/.zshrc
  echo add path to `pwd` to .zhshrc
fi
if [ -f $HOME/.bashrc ]; then
  echo export PATH=\$PATH:`pwd`/bin >> $HOME/.bashrc
  echo add path to `pwd` to .bashrc
fi
