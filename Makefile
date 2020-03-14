SHELL:=/bin/bash

init-dev:
{
	source venv/bin/activate || (python3 -m venv venv && source venv/bin/activate);
}
