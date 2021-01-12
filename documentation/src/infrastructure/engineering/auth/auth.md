# Authentication and Authorization

<p align="center">
    <img alt="Signin and Registration" src="./signin-registration.drawio.svg">
</p>
<p align="center">
    <img alt="Endpoint Authorization" src="./endpoint-authorization.drawio.svg">
</p>

# Single Signin

Single Signin is accomplished by:

1. Taking advantage of the pre-existing auth flow on the main site
2. Implementing a signin flow on the secondary site
3. Leveraging the fact that [GET / read-only requests are secure](https://security.stackexchange.com/questions/115794/should-i-use-csrf-protection-for-get-requests) despite not having the CSRF header check.

In more detail, the flow is like this (assuming user has already signed in on the main auth site):

1. Client makes a GET request to the main auth server's endpoint
2. Main Auth Server validates the cookie (and _only_ the cookie)
3. Main Auth Server generates and responds with a new Signin JWT (without a CSRF token)
4. Client passes this JWT along to Secondary Auth Server's login endpoint
5. Secondary Auth Server validates the JWT (via a validation request to Main Auth Server)
6. Secondary Auth Server follows the same signin flow as the Main Auth Server

Essentially, what's happening is that the user is really signing into the secondary site just like the main one. However, instead of actually entering their credentials, their credentials are proven, re-used, and supplied via the main auth site.

# Session Expirey

The cookie is set for some amount of time (currently preconfigured at 2 weeks)

It can be extended at any point - however, real extending with completely fresh data would mean resetting the CSRF token as well.

Therefore two services should be exposed:

1. `extend-auth-soft` - will re-use the existing CSRF token and simply re-set the cookie with an updated max-age

2. `extend-auth-hard` - will also change the CSRF token and the client is expected to replace it in local storage

The idea is that it's simple for a website to fire `extend-auth-soft` as some Future/Promise on startup and not need to worry about handling the response