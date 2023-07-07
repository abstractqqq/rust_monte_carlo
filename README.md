# In this Repo...

We are going to explore the following question regarding the number of estimators to use for a random forest model. The question statement is:

Given a random forest model, we want to find a good choice for the n_estimator parameter.

We are going to subsample on the feature space for each decision tree in the forest.

Let's say we have 100 features in total. And we are subsampling 60 features for each decision tree. Ignore depth for now. How many estimators do we need so that we have 95% confidence that each feature is selected at least 3 times? (by different trees)

The answer is an elegant monte carlo simulation, with Rust + Rayon (multhreading) we are able to get results in 36ms, compared to 400ms using Python + NumPy on my personal computer.
