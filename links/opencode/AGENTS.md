# Global instructions

## Publishing to remotes

Never run `git push`, or any other command that writes to a remote, unless the
user has explicitly asked for that specific push in the current conversation.

- Approval covers one push only. Never reuse an earlier approval for a
  different branch, repository, or a later point in the session.
- When approval has not been given, ask and wait. Do not run the command and ask
  afterwards.
- Committing, staging, amending, and rebasing are fine without asking. Only
  publishing to a remote requires approval.

The `git push *` shell permission in `opencode.json` already enforces this and
prompts on its own. Do not try to work around that prompt.