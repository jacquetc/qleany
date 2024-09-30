// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_brand_query_handler.h"
#include "repository/interface_brand_repository.h"
#include "tools/automapper.h"

using namespace FrontEnds;
using namespace FrontEnds::Application::Features::Brand::Queries;

GetBrandQueryHandler::GetBrandQueryHandler(InterfaceBrandRepository *repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered) {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<BrandDTO> GetBrandQueryHandler::handle(QPromise<Result<void>> &progressPromise, const GetBrandQuery &query)
{
    Result<BrandDTO> result;

    try {
        result = handleImpl(progressPromise, query);
    } catch (const std::exception &ex) {
        result = Result<BrandDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetBrandQuery:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<BrandDTO> GetBrandQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise, const GetBrandQuery &query)
{
    qDebug() << "GetBrandQueryHandler::handleImpl called with id" << query.id;

    // do
    auto brandResult = m_repository->get(query.id);

    QLN_RETURN_IF_ERROR(BrandDTO, brandResult)

    // map
    auto dto = FrontEnds::Tools::AutoMapper::map<FrontEnds::Entities::Brand, BrandDTO>(brandResult.value());

    qDebug() << "GetBrandQueryHandler::handleImpl done";

    return Result<BrandDTO>(dto);
}

bool GetBrandQueryHandler::s_mappingRegistered = false;

void GetBrandQueryHandler::registerMappings()
{
    FrontEnds::Tools::AutoMapper::registerMapping<FrontEnds::Entities::Brand, Contracts::DTO::Brand::BrandDTO>(true, true);
}