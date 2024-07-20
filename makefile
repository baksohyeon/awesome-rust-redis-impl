
test:
	codecrafters test

submit:
	git stash && git checkout master && git pull && git stash apply && \
	codecrafters submit


kill:
	kill -9 $(shell lsof -t -i:6379)



