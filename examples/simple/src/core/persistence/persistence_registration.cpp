// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "persistence_registration.h"
#include <qleany/database/database_context.h>
#include <qleany/database/database_table_group.h>

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

    auto *brandDatabaseTableGroup = new DatabaseTableGroup<Simple::Entities::Brand>(context);
    auto *carDatabaseTableGroup = new DatabaseTableGroup<Simple::Entities::Car>(context);
    auto *clientDatabaseTableGroup = new DatabaseTableGroup<Simple::Entities::Client>(context);
    auto *passengerDatabaseTableGroup = new DatabaseTableGroup<Simple::Entities::Passenger>(context);

    Result<void> initResult = context->init();

    if (initResult.hasError())
    {
        Error error = initResult.error();
        qCritical() << error.className() + QString::fromLatin1("\n") + error.code() + QString::fromLatin1("\n") +
                           error.message() + QString::fromLatin1("\n") + error.data();
    }

    // repositories:

    BrandRepository *brandRepository = new BrandRepository(brandDatabaseTableGroup);
    PassengerRepository *passengerRepository = new PassengerRepository(passengerDatabaseTableGroup);
    ClientRepository *clientRepository = new ClientRepository(clientDatabaseTableGroup, passengerRepository);
    CarRepository *carRepository = new CarRepository(carDatabaseTableGroup, brandRepository, passengerRepository);

    // register repositories:

    RepositoryProvider::instance()->registerRepository("brand", brandRepository);
    RepositoryProvider::instance()->registerRepository("car", carRepository);
    RepositoryProvider::instance()->registerRepository("client", clientRepository);
    RepositoryProvider::instance()->registerRepository("passenger", passengerRepository);
}

RepositoryProvider *PersistenceRegistration::repositoryProvider()
{
    return RepositoryProvider::instance();
}
