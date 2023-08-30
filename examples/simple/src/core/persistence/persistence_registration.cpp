// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "persistence_registration.h"
#include "qleany/database/database_context.h"
#include "qleany/database/database_table_group.h"
#include "qleany/repository/repository_provider.h"

#include "repository/brand_repository.h"
#include "repository/car_repository.h"
#include "repository/client_repository.h"
#include "repository/passenger_repository.h"

using namespace Qleany;
using namespace Qleany::Database;
using namespace Qleany::Repository;
using namespace Simple::Persistence;
using namespace Simple::Persistence::Repository;

PersistenceRegistration::PersistenceRegistration(QObject *parent) : QObject{parent}
{
    QSharedPointer<DatabaseContext> context(new DatabaseContext());

    // database tables:

    auto *passengerDatabaseTableGroup = new DatabaseTableGroup<Simple::Domain::Passenger>(context);
    auto *brandDatabaseTableGroup = new DatabaseTableGroup<Simple::Domain::Brand>(context);
    auto *clientDatabaseTableGroup = new DatabaseTableGroup<Simple::Domain::Client>(context);
    auto *carDatabaseTableGroup = new DatabaseTableGroup<Simple::Domain::Car>(context);

    Result<void> initResult = context->init();

    if (initResult.hasError())
    {
        Error error = initResult.error();
        qCritical() << error.className() + "\n" + error.code() + "\n" + error.message() + "\n" + error.data();
    }

    // repositories:

    PassengerRepository *passengerRepository = new PassengerRepository(passengerDatabaseTableGroup);
    BrandRepository *brandRepository = new BrandRepository(brandDatabaseTableGroup);
    ClientRepository *clientRepository = new ClientRepository(clientDatabaseTableGroup, passengerRepository);
    CarRepository *carRepository = new CarRepository(carDatabaseTableGroup, brandRepository, passengerRepository);

    // register repositories:

    RepositoryProvider::instance()->registerRepository("passenger", passengerRepository);
    RepositoryProvider::instance()->registerRepository("brand", brandRepository);
    RepositoryProvider::instance()->registerRepository("client", clientRepository);
    RepositoryProvider::instance()->registerRepository("car", carRepository);
}

RepositoryProvider *PersistenceRegistration::repositoryProvider()
{
    return RepositoryProvider::instance();
}
