// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_passenger_query_handler.h"
#include "repository/interface_passenger_repository.h"
#include "tools/automapper.h"

using namespace FrontEnds;
using namespace FrontEnds::Application::Features::Passenger::Queries;

GetPassengerQueryHandler::GetPassengerQueryHandler(InterfacePassengerRepository *repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered) {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<PassengerDTO> GetPassengerQueryHandler::handle(QPromise<Result<void>> &progressPromise, const GetPassengerQuery &query)
{
    Result<PassengerDTO> result;

    try {
        result = handleImpl(progressPromise, query);
    } catch (const std::exception &ex) {
        result = Result<PassengerDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetPassengerQuery:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<PassengerDTO> GetPassengerQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise, const GetPassengerQuery &query)
{
    qDebug() << "GetPassengerQueryHandler::handleImpl called with id" << query.id;

    // do
    auto passengerResult = m_repository->get(query.id);

    QLN_RETURN_IF_ERROR(PassengerDTO, passengerResult)

    // map
    auto dto = FrontEnds::Tools::AutoMapper::map<FrontEnds::Entities::Passenger, PassengerDTO>(passengerResult.value());

    qDebug() << "GetPassengerQueryHandler::handleImpl done";

    return Result<PassengerDTO>(dto);
}

bool GetPassengerQueryHandler::s_mappingRegistered = false;

void GetPassengerQueryHandler::registerMappings()
{
    FrontEnds::Tools::AutoMapper::registerMapping<FrontEnds::Entities::Passenger, Contracts::DTO::Passenger::PassengerDTO>(true, true);
}