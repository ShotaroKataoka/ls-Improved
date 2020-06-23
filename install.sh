if [ -f $HOME/.zshrc ]; then
  echo export PATH=\$PATH:`pwd` >> $HOME/.zshrc
  echo add path to `pwd` to .zhshrc
  source $HOME/.zshrc
fi
if [ -f $HOME/.bashrc ]; then
  echo export PATH=\$PATH:`pwd` >> $HOME/.bashrc
  echo add path to `pwd` to .bashrc
  source $HOME/.bashrc
fi
