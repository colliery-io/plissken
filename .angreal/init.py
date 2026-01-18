import os
import stat
import subprocess

import angreal
from angreal.replay import Replay
from angreal import VirtualEnv
from angreal import Git
from angreal import GitLab


here = os.path.dirname(__file__)

pypirc = os.path.expanduser(os.path.join('~', '.pypirc'))
requirements = os.path.join(here, '..', 'requirements', 'dev.txt')
setup_py = os.path.join(here, '..', 'setup.py')


@angreal.command()
@angreal.option('--gitlab_token', envvar='GITLAB_TOKEN', help='gitlab token to use (default = $GITLAB_TOKEN)')
@angreal.option('--no_pypi', is_flag=True, help='do not setup a pypi registry')
@angreal.option('--no_gitlab', is_flag=True, help='no gitlab project created')
def init(gitlab_token, no_pypi, no_gitlab):
    """
    initialize a python project
    """

    # Get our replay
    replay = Replay()

    # create our virtual environment and activate it for the rest of this run.

    angreal.warn('Virtual environment {} being created.'.format('plissken'))
    VirtualEnv(name='plissken', python='python3', requirements=requirements)
    angreal.win('Virtual environment {} created'.format('plissken'))

    if not no_pypi:  # double negative, skipped if no_pypi == True
            # Get a pypirc setup if one doesn't exist.
        if not os.path.isfile(pypirc):
            rv = input('No pypirc file detected do you wish to continue upload ? y/[n]')
            if rv == 'y':
                pypi_un = input('What is your pypi user name ?')
                pypi_pw = input('What is your pypi password ?')
                with open(pypirc, 'w') as f:
                    print('[server-login]', file=f)
                    print('username: {}'.format(pypi_un), file=f)
                    print('password: {}'.format(pypi_pw), file=f)
                os.chmod(pypirc, stat.S_IRUSR)

        # Register with a 0.0.0 upload

        pypi_rv = subprocess.run('python {} bdist_wheel upload'.format(setup_py),
                                 shell=True, stderr=subprocess.STDOUT)

        if pypi_rv.returncode == 0:
            angreal.win('plissken version 0.0.0-prerelease reserved on pypi.')
        else:
            angreal.error('Failed to register with PyPi')
            angreal.warn('`python setup.py bdist_wheel upload` can be run later if you wish.')

    else:
        angreal.warn('No pypi registry reserved, use `python setup.py bdist_wheel upload` later if you wish.')

    if not no_gitlab:
        # Now go get our Gitlab project
        gl = GitLab('https://gitlab.com', access_token=gitlab_token)

        # Get our namespace
        requested_namespace = replay.get('namespace')

        if requested_namespace != 'user':
            gl.create_repository(replay.get('name'), namespace=requested_namespace)
            angreal.win('{} created'.format(gl.repo.ssh_url_to_repo))
        else:
            gl.create_repository(replay.get('name'))
            angreal.win('{} created'.format(gl.repo.ssh_url_to_repo))

        gl.enable_gitlfs()
        gl.enable_issues()
        gl.enable_pipelines()

    # Initialize the git repo
    git = Git()
    git.init()
    git.add('.')

    if not no_gitlab:  # and push to the remote
        git.remote('add', 'origin', gl.repo.ssh_url_to_repo)
        git.commit('-m', 'Project initialized via angreal.')
        git.push('origin', 'master')

    angreal.win('plissken successfully created !')

    return
