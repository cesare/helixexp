# Helixexp

This is an experiment of building a rubygem with the Rust native extension using Helix.

## Preparation

Before building, you have to [install Rust](https://www.rust-lang.org/en-US/install.html).
And you need Matz' Ruby Implementation.

## Building

To build a gem,

```bash
$ bundle exec rake build
```

You will find the gem `pkg/helixexp-x.x.x.gem`

To run the code,

```bash
$ ./bin/console
```

and

```ruby
Helixexp.greet("world")
# => prints "hello, world"
```

## Development

After checking out the repo, run `bin/setup` to install dependencies. You can also run `bin/console` for an interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`. To release a new version, update the version number in `version.rb`, and then run `bundle exec rake release`, which will create a git tag for the version, push git commits and tags, and push the `.gem` file to [rubygems.org](https://rubygems.org).

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/[USERNAME]/helixexp. This project is intended to be a safe, welcoming space for collaboration, and contributors are expected to adhere to the [Contributor Covenant](http://contributor-covenant.org) code of conduct.

## Code of Conduct

Everyone interacting in the Helixexp project’s codebases, issue trackers, chat rooms and mailing lists is expected to follow the [code of conduct](https://github.com/[USERNAME]/helixexp/blob/master/CODE_OF_CONDUCT.md).
